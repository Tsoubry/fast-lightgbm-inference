from typing import Any, Dict
import os
import json

import httpx

from odt.config import PipeConfig

_TEMPFILENAME = "lgbm_tmp_model.txt"

class ODTManager:

    def __init__(self, server_host: str) -> None:
        self.server_host = server_host

    def update_config(self, config: PipeConfig):
        # serialization from pydandic with .json method doesn't work internally
        json_data = json.loads(config.json())
        r = httpx.post(f"{self.server_host}/config", json=json_data)

        if r.status_code == 200:
            print("Updating config succeeded!")
        else:
            raise Exception(f"Something went wrong updating the config, status code {r.status_code}")

    def update_model(self, model: Any):

        model.save_model(_TEMPFILENAME)

        with open(_TEMPFILENAME, "r") as f:
            in_mem_model: str = f.read()
        
        os.remove(_TEMPFILENAME) 

        r = httpx.post(f"{self.server_host}/model", content=bytes(in_mem_model, encoding='utf-8'))

        if r.status_code == 200:
            print("Updating model succeeded!")
        else:
            raise Exception(f"Something went wrong updating the model, status code {r.status_code}")
        

    def update_config_and_model(self, config: PipeConfig, model: Any):
        self.update_config(config)
        self.update_model(model)


    def get_prediction(self, data: Dict[str, Any]) -> float:
        r = httpx.post(f"{self.server_host}/predict", json=data)
        return r.json()
        