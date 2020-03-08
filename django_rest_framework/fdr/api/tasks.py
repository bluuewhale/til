from collections import defaultdict

import pandas as pd

import FinanceDataReader as fdr


class DataLoader:
    """ FDR Data Loader"""

    def __init__(self):
        pass

    def __load_ohlc(self, ticker, sdate=None, edate=None):

        df = fdr.DataReader(ticker, sdate, edate)
        df.columns = list(map(lambda x: x.lower(), df.columns))
        df["ticker"] = ticker
        df["date"] = list(map(lambda x: x.strftime("%Y-%m-%d"), df.index))

        return df.to_dict("list")

    def load_ohlc(self, ticker, sdate=None, edate=None):
        """ load OHLCVC(Open, High, Low, CLose, Volume, Change)

        Parameters
        ----------
        ticker : str
        sdate : str
            start date, if None: from 1980-01-01
            format="YYYYMMDD", default=None
        edate : str
            end date, if None: til today
            format="YYYYMMDD", default=None

        Returns
        -------
        dict
            OHLCVC(Open, High, Low, CLose, Volume, Change)
        """

        if (sdate is None) and (edate is None):

            data_dict = self.merge_dict(
                self.load_ohlc(ticker, "1980-01-01", "1994-12-31"),
                self.load_ohlc(ticker, "1995-01-01", "2010-12-31"),
                self.load_ohlc(ticker, "2010-01-01", "2019-12-31"),
            )

        else:
            data_dict = self.__load_ohlc(ticker, sdate, edate)

        return data_dict

    # utility
    @staticmethod
    def merge_dict(*dicts):

        result = defaultdict(lambda: [])

        for dict_ in dicts:
            keys = dict_.keys()

            for key in keys:
                result[key] += list(dict_.get(key))

        return result
