from typing import Any

from locust import task, FastHttpUser
import gevent

import pandas as pd
import numpy as np
from scipy.io.arff import loadarff 

COLUMNS = ['age', 'menopause', 'tumor-size', 'inv-nodes', 'node-caps', 'deg-malig', 'breast', 'breast-quad', 'irradiat']
SIZE = 10

def get_data_values() -> dict[str, np.ndarray]:
    raw_data = loadarff('../examples/breast-cancer/dataset_13_breast-cancer.arff')
    df = pd.DataFrame(raw_data[0])
    for c in list(df.columns):
        df[c] = df[c].str.decode('utf-8')

    vals = {}
    for column in COLUMNS:
        vals[column] = df[column].unique()

    return vals


unique_values = get_data_values()


class LoadTest(FastHttpUser):

    @task
    def predict_test(self):

        def concurrent_request(data: dict[str, Any]):
            self.client.post('/predict', json=data)

        pool = gevent.pool.Pool()
        choice_dict = { 
            c: np.random.choice(unique_values[c], size=SIZE)
            for c in COLUMNS
        }

        for i in range(SIZE):
            features = {
                c: choice_dict[c][i]
                for c in COLUMNS
            }

            pool.spawn(concurrent_request, features)

        pool.join()
