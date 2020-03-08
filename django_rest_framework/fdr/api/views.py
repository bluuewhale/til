from datetime import datetime as dt
from datetime import timedelta

import FinanceDataReader as fdr
import pandas as pd
from django.http import HttpResponse, JsonResponse
from rest_framework.decorators import api_view

from .serializer import OHLCSerializer
from .models import OHLC
from .tasks import DataLoader


@api_view(["GET"])
def get_ohlc(request, ticker):
    """ ticker로 필터링 """

    model = OHLC
    serializer_cls = OHLCSerializer
    loader = DataLoader()

    if request.method == "GET":

        # get data from db
        queryset = model.objects.filter(ticker=ticker).order_by("date")
        serializer = serializer_cls(queryset, many=True)
        ohlc_dict = serializer.data

        if len(ohlc_dict) == 0:  # if db is empty, load new data from fdr
            ohlc_dict = loader.load_ohlc(ticker)

            if len(ohlc_dict["date"]) == 0:  # check if newly loaded fdr data is empty
                return HttpResponse("{} data doesn't exist".format(ticker), status=400)

            # TODO: save new fdr data
            # post_serializer = serializer_cls(data=ohlc_dict)

            # if post_serializer.is_valid():
            #    return HttpResponse("200", status=200)
            # else:
            #    return JsonResponse(post_serializer.errors)

        date_format = "%Y-%m-%d"
        last_date = ohlc_dict["date"][-1]
        today = dt.now().strftime(date_format)

        if today == last_date:  # data is up to date

            # if today > last_date:  # if need update, do it

            sdate = dt.strptime(last_date, date_format) + timedelta(days=1)
            sdate = sdate.strftime(date_format)
            edate = today

            update_data = loader.load_ohlc(ticker, sdate, edate)

            # TODO: update_data DB에 추가하는 코드 작성

            ohlc_dict = loader.merge_dict(ohlc_dict, update_data)

        return JsonResponse(ohlc_dict, safe=False)
