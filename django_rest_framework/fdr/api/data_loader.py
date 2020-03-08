import FinanceDataReader as fdr
from .utility import merge_dict


class DataLoader:
    """ FDR Data Loader"""

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
            dictionary of list(dl)
            OHLCVC(Open, High, Low, CLose, Volume, Change)
        """

        if (sdate is None) and (edate is None):

            data_dict = merge_dict(
                self.__load_ohlc(ticker, "1980-01-01", "1994-12-31"),
                self.__load_ohlc(ticker, "1995-01-01", "2009-12-31"),
                self.__load_ohlc(ticker, "2010-01-01"),
            )

        else:
            data_dict = self.__load_ohlc(ticker, sdate, edate)

        return data_dict

    def __load_ohlc(self, ticker, sdate=None, edate=None):

        try:
            df = fdr.DataReader(ticker, sdate, edate)
        except IndexError:  # empty
            return {}

        df.columns = list(map(lambda x: x.lower(), df.columns))
        df["ticker"] = ticker
        df["date"] = list(map(lambda x: x.strftime("%Y-%m-%d"), df.index))

        return df.to_dict("list")
