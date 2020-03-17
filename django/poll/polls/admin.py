from django.contrib import admin
from .models import Question, Choice


class ChoiceInline(admin.TabularInline):
    model = Choice
    extra = 3


class QuestionAdmin(admin.ModelAdmin):
    # fields = ["pub_date", "text"]
    list_display = ["text", "pub_date", "was_published_recently"]
    list_filter = ["pub_date"]
    search_fields = ["text"]
    fieldsets = [
        (None, {"fields": ["text"]}),
        ("Date information", {"fields": ["pub_date"], "classes": ["collapse"]}),
    ]
    inlines = [ChoiceInline]


admin.site.register(Question, QuestionAdmin)
admin.site.register(Choice)
