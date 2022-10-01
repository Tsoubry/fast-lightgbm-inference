from typing import Optional

import pandas as pd
import numpy as np

from sklearn.base import BaseEstimator, TransformerMixin

from odt._odt import transform_i64, transform_object, transform_nullable_object
from odt.config import PipeConfig, Feature, DataType

__all__ = ["transform_i64", "transform_object", "transform_nullable_object"]

class ODTTransformer(BaseEstimator, TransformerMixin):

    def __init__(self, config: Optional[PipeConfig] = None) -> None:
        super().__init__()
        self.config: Optional[PipeConfig] = config

    def _auto_configure(self, df: pd.DataFrame):
        self.config = PipeConfig()

        for column in list(df.columns):
            if df[column].dtype == np.float64:
                self.config.add_feature(Feature(name=column, datatype=DataType.Float, nullable=False))
            elif df[column].dtype == np.int64:
                self.config.add_feature(Feature(name=column, datatype=DataType.Long, nullable=False))
            elif df[column].dtype == object:
                self.config.add_feature(Feature(name=column, datatype=DataType.Object, nullable=True))
            else:
                raise Exception(f"Column {column} with datatype {df[column].dtype} not supported!")

    def _check_configuration(self, df: pd.DataFrame):
        for column in list(df.columns):
            if column not in self.config.feature_names:
                raise Exception(f"Column {column} not found in data, but required for config")
            
        for feature in self.config.features:
            if feature.datatype == DataType.Float:
                if df[feature.name].dtype != np.float64:
                    raise Exception(f"Column {column} is data type {df[feature.name].dtype}, but needs to be np.float64!")
            elif feature.datatype == DataType.Long:
                if df[feature.name].dtype != np.int64:
                    raise Exception(f"Column {column} is data type {df[feature.name].dtype}, but needs to be np.int64!")
            else:
                if df[feature.name].dtype != object:
                    raise Exception(f"Column {column} is data type {df[feature.name].dtype}, but needs to be object!")

    def fit(self, X: pd.DataFrame, y=None):
        if self.config is None:
            self._auto_configure(X)
        else:
            self._check_configuration(X)

        return self

    def transform(self, X: pd.DataFrame, y=None) -> np.ndarray:
        # Perform transformations
        new_df = pd.DataFrame()

        for feature in self.config.features:
            if feature.datatype == DataType.Float:
                new_df[feature.name] = X[feature.name]
            elif feature.datatype == DataType.Long:
                new_df[feature.name] = transform_i64(X[feature.name].values)
            elif (feature.datatype == DataType.Object) & (feature.nullable):
                (null_c, value_c) = transform_nullable_object(X[feature.name].values.astype('O'))
                new_df[f'null_{feature.name}'] = null_c
                new_df[feature.name] = value_c
            else:
                new_df[feature.name] = transform_object(X[feature.name].values.astype('O'))

        return new_df.values

