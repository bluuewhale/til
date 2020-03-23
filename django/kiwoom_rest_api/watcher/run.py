import asyncio
import sys
from PyQt5.QtWidgets import QApplication

from kiwoom_api.api import Kiwoom, DataFeeder, Executor
from watchers import TrWatcher, OrderWatcher


def main():
    app = QApplication(sys.argv)

    kiwoom = Kiwoom.instance()
    kiwoom.commConnect()
    feeder = DataFeeder(kiwoom)
    executer = Executor(kiwoom)

    tr_watcher = TrWatcher(kiwoom, feeder)
    order_watcher = OrderWatcher(kiwoom, executer)

    loop = asyncio.get_event_loop()
    coroutineList = [
        tr_watcher.run(main_func_name="request"),
        order_watcher.run(main_func_name="sendOrder"),
    ]
    loop.run_until_complete(asyncio.gather(*coroutineList))


if __name__ == "__main__":
    main()
