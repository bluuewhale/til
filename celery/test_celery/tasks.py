import random
import os
import time

from test_celery.celery import app


@app.task
def working(num: int):
    # print("long time task begins")
    time.sleep(10)
    print(f"PID:{os.getpid()}, {num} Done")
    # print("long time task finished")
    # return "11111"
