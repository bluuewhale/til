import itertools

import FinanceDataReader as fdr


class DataLoader:
    """ FDR Data Loader"""

    @staticmethod
    def load_ohlc(ticker, sdate=None, edate=None):
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
        list
            ld (list of dictionaries)
            [{Open, High, Low, CLose, Volume, Change}]
        """

        def _load_ohlc(ticker, sdate=None, edate=None):

            try:
                df = fdr.DataReader(ticker, sdate, edate)
            except (IndexError, ValueError):  # date is wrong, symbol not found
                return []

            df["date"] = [t.strftime("%Y-%m-%d") for t in df.index]
            df["ticker"] = ticker
            df.columns = [c.lower() for c in df.columns]

            return df.to_dict("record")

        if (sdate is None) and (edate is None):
            data = list(
                itertools.chain(
                    _load_ohlc(ticker, "1980-01-01", "1994-12-31"),
                    _load_ohlc(ticker, "1995-01-01", "2009-12-31"),
                    _load_ohlc(ticker, "2010-01-01"),
                )
            )
        else:
            data = _load_ohlc(ticker, sdate, edate)

        return data
