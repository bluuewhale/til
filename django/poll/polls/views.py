from django.shortcuts import render, get_object_or_404
from django.http import HttpResponse, HttpResponseRedirect
from django.urls import reverse
from .models import Question, Choice

response = "Question {} : {}"


def index(request):
    questions = Question.objects.order_by("-pub_date")
    return render(request, "polls/index.html", {"questions": questions})


def detail(request, id):
    question = get_object_or_404(Question, pk=id)
    return render(request, "polls/detail.html", {"question": question})


def results(request, id):
    question = get_object_or_404(Question, pk=id)
    return render(request, "polls/results.html", {"question": question})


def vote(request, id):
    question = get_object_or_404(Question, pk=id)
    try:
        selected_choice = question.choice_set.get(pk=request.POST["choice"])
    except (KeyError, Choice.DoesNotExist):
        return render(
            request,
            "polls/detail.html",
            {"question": question, "error_message": "You didn't select a choice"},
        )

    selected_choice.votes += 1
    selected_choice.save()
    return HttpResponseRedirect(reverse("polls:results", args=(question.id,)))

