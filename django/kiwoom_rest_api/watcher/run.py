import sys
from PyQt5.QtWidgets import QApplication

from kiwoom_api.api import Kiwoom, DataFeeder
from watchers import DataRequestWatcher

if __name__ == "__main__":
    app = QApplication(sys.argv)

    kiwoom = Kiwoom()
    kiwoom.commConnect()
    feeder = DataFeeder(kiwoom)

    data_watcher = DataRequestWatcher(kiwoom, feeder)
    data_watcher.run()