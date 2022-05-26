import dataclasses
import typing
import datetime


@dataclasses.dataclass
class Semaphore:
    """
    Use to control access counts for a resource
    with a given period
    """
    access_times: int
    per_period: datetime.timedelta

    def calc_delay(self) -> float:
        """
        Calculate delay a task/thread should sleep
        """
        pass