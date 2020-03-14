import asyncio
import sys
from PyQt5.QtWidgets import QApplication

from kiwoom_api.api import Kiwoom, DataFeeder, Executor
from watchers import TrWatcher, OrderWatcher

def main():
    app = QApplication(sys.argv)

    kiwoom = Kiwoom()
    kiwoom.commConnect()
    feeder = DataFeeder(kiwoom)
    executer = Executor(kiwoom)

    tr_watcher = TrWatcher(kiwoom, feeder)
    order_watcher = OrderWatcher(kiwoom, executer)
    
    loop = asyncio.get_event_loop()
    coroutineList = [
        tr_watcher.asyncRun(),
        order_watcher.asyncRun(),
    ]
    loop.run_until_complete(asyncio.gather(*coroutineList))
    
if __name__ == "__main__":
    main()