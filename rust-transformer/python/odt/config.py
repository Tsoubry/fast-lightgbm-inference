from enum import Enum
from typing import Optional

from pydantic import BaseModel, Field


class DataType(str, Enum):
    Float: str = "Float"
    Long: str = "Long"
    Object: str = "Object"


class Feature(BaseModel):
    name: str
    datatype: DataType
    nullable: bool


class PipeConfig(BaseModel):
    features: list[Feature] = Field(list())

    def _sort_features(self):
        self.features = sorted(self.features, key=lambda x: x.name)

    def __post_init_post_parse__(self):
        self._sort_features()

    def add_feature(
        self,
        feature: Optional[Feature] = None,
        name: Optional[str] = None,
        datatype: Optional[str] = None,
        nullable: bool = False,
    ):

        if feature is not None:
            self.features.append(feature)
        else:
            if (name != None) & (datatype != None) & (nullable != None):
                self.features.append(
                    Feature(name=name,
                            datatype=DataType(datatype),
                            nullable=nullable))
            else:
                raise Exception(
                    "Adding a feature requires keywords name, datatype and nullable to be defined!"
                )

        self._sort_features()

    @property
    def feature_names(self) -> list[str]:
        return list(map(lambda x: x.name, self.features))
