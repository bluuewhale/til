from django.contrib import admin

from .models import Order


class OrderAdmin(admin.ModelAdmin):
    list_display = (
        "user",
        "code",
        "qty",
        "orderType",
        "price",
        "hogaType",
        "is_success",
        "time",
        "msg",
    )


admin.site.register(Order, OrderAdmin)
