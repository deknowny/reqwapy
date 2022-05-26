import asyncio
import threading
import dataclasses
import time
import datetime

import pytest
import ralipyard


@dataclasses.dataclass
class SharedCounter:
    done: int = 0


def thread_task(sem: ralipyard.Semaphore, counter: SharedCounter):
    delay = sem.calc_delay()
    time.sleep(delay)
    counter.done += 1


async def asyncio_task(sem: ralipyard.Semaphore, counter: SharedCounter):
    delay = sem.calc_delay()
    await asyncio.sleep(delay)
    counter.done += 1


def test_threading_access():
    sem = ralipyard.Semaphore(
        access_times=5,
        per_period=datetime.timedelta(seconds=1)
    )
    counter = SharedCounter()

    threads = [
        threading.Thread(target=thread_task, args=[sem, counter])
        for _ in range(15)
    ]

    [thread.start() for thread in threads]
    time.sleep(1.1)
    assert counter.done == 5

    time.sleep(1.1)
    assert counter.done == 10

    [thread.join() for thread in threads]


@pytest.mark.asyncio
async def test_asyncio_access():
    sem = ralipyard.Semaphore(
        access_times=10,
        per_period=datetime.timedelta(seconds=0.5)
    )
    counter = SharedCounter()

    tasks = [
        asyncio.create_task(asyncio_task(sem, counter))
        for _ in range(30)
    ]

    await asyncio.sleep(0.6)
    assert counter.done == 10

    await asyncio.sleep(0.6)
    assert counter.done == 20

    await asyncio.gather(*tasks)
