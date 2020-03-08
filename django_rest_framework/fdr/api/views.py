from collections import defaultdict
from datetime import datetime as dt
from datetime import timedelta

from django.db.utils import IntegrityError
from django.http import HttpResponse, JsonResponse
from rest_framework.decorators import api_view

import FinanceDataReader as fdr
import numpy as np
import pandas as pd


from .serializer import OHLCReadSerializer, OHLCCreateSerializer
from .models import OHLC
from .data_loader import DataLoader
from .utility import dl_to_ld, ld_to_dl, merge_dict


@api_view(["GET"])
def get_ohlc(request, ticker):

    model = OHLC
    read_serializer_cls = OHLCReadSerializer
    create_serializer_cls = OHLCCreateSerializer
    loader = DataLoader()

    def create(data_dl):
        """ FDR에서 불러온 OHLC 데이터를 instance로 DB에 저장하는 함수입니다.

        Parameters
        ----------
        data_dl : dict
            dictonary of lists

        Returns
        -------
        django.http.Response
        """

        # convert dict of lists to list of dicts
        ld = dl_to_ld(data_dl)
        create_serializer = create_serializer_cls(data=ld, many=True)

        # create new instance in DB
        if create_serializer.is_valid():
            try:
                create_serializer.save()
            except IntegrityError:
                pass
        else:
            return JsonResponse(create_serializer.errors, safe=False, status=400)

        return HttpResponse(status=201)

    if request.method == "GET":
        """ Data Response Process

        1. check if data in DB
            True: move to 2
            False: move to 3

        2. load full-period data from FDR
            create instance in DB -> return data

        3. check if update is needed
            True: load updated data -> create instance in DB -> return data
            False: return data
        """

        # 1. get data from db
        queryset = model.objects.filter(ticker=ticker).order_by("date")
        serializer = read_serializer_cls(queryset, many=True)
        data = serializer.data

        if len(data):  # if data in DB, convert type for data handling
            ohlc_dl = ld_to_dl(data)

        else:  # 2. load new data from fdr
            ohlc_dl = loader.load_ohlc(ticker)

            if not len(ohlc_dl.get("date", [])):  # if fdr data is empty, ERROR
                return HttpResponse("{} data doesn't exist".format(ticker), status=400)

            # create instance in DB (save) and receive create response
            create_res = create(ohlc_dl)
            if create_res.status_code != 201:
                return create_res

        # 3. check if need update
        format = "%Y-%m-%d"  # YYYY-MM-DD
        last_update = ohlc_dl["date"][-1]
        today = dt.now().strftime(format)

        if np.busday_count(last_update, today):  # if need update, do it!

            sdate = dt.strptime(last_update, format) + timedelta(days=1)
            sdate = sdate.strftime(format)  # last_date in DB + 1day
            edate = today

            update_dl = loader.load_ohlc(ticker, sdate, edate)  # load data

            if len(update_dl.get("date", [])):  # update only when available
                ohlc_dl = merge_dict(ohlc_dl, update_dl)

                create_res = create(update_dl)  # create instance in DB
                if create_res.status_code != 201:
                    return create_res

        return JsonResponse(ohlc_dl, safe=False)
