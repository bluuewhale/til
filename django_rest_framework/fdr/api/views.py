from datetime import datetime as dt
from datetime import timedelta

from django.db.utils import IntegrityError
from django.http import HttpResponse, JsonResponse
from rest_framework.decorators import api_view

import FinanceDataReader as fdr
import numpy as np

from .serializer import OHLCSerializerFactory
from .models import OHLC
from .data_loader import DataLoader
from .utility import ld_to_dl


def save(data):
    """ FDR에서 불러온 OHLC data를 instance로 DB에 저장하는 함수입니다.

    Parameters
    ----------
    data : list
        list of dictionaries

    Returns
    -------
    django.http.Response
    """

    # convert dict of lists to list of dicts
    serializer_cls = OHLCSerializerFactory.create()
    serializer = serializer_cls(
        data=ld_to_dl(data), many=True
    )  # ld to dl for saving data

    # create new instance in DB
    if serializer.is_valid():
        try:
            serializer.save()
        except IntegrityError:
            pass
    else:
        return HttpResponse(status=400)

    return HttpResponse(status=201)


@api_view(["GET"])
def get_ohlc(request):
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

    if request.method == "GET":

        model = OHLC
        serializer_cls = OHLCSerializerFactory.create()

        # 1. query data fr2om db
        url_kwargs = request.GET.dict()
        queryset = model.objects.filter(**url_kwargs)
        serializer = serializer_cls(queryset, many=True)
        data = serializer.data  # list of dicts

        ticker = url_kwargs.get("ticker", "")
        # 2. # if no data in DB, load new data from fdr
        if not data:
            data = DataLoader.load_ohlc(ticker)

            if not data:  # if fdr data is empty, ERROR
                return HttpResponse("wrong ticker", status=400)

            crt_serializer = serializer_cls(data=data, many=True)
            if crt_serializer.is_valid():
                try:
                    crt_serializer.save()
                except IntegrityError:
                    pass
            else:
                # return HttpResponse("validation failed", status=400)
                return JsonResponse(crt_serializer.errors, safe=False)  # debug only

        # 3. check if need update
        fmt = "%Y-%m-%d"  # YYYY-MM-DD
        last_update = data[-1].get("date")
        today = dt.now().strftime(fmt)

        if np.busday_count(last_update, today):  # if need update, do it!

            sdate = (dt.strptime(last_update, fmt) + timedelta(days=1)).strftime(fmt)
            edate = today
            update_data = DataLoader.load_ohlc(ticker, sdate, edate)  # load data

            if update_data:  # update only when available
                crt_serializer = serializer_cls(data=update_data, many=True)

                if crt_serializer.is_valid():
                    try:
                        crt_serializer.save()
                    except IntegrityError:
                        pass
                else:
                    # return HttpResponse("validation failed", status=400)
                    return JsonResponse(crt_serializer.errors, safe=False)  # debug

                data += update_data  # concat

        return JsonResponse(data, safe=False)
