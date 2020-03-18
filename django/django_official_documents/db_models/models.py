from django.db import models

""" Fields """


class Person(models.Model):
    # char field must have max_length argument
    name = models.CharField(max_length=50)

    def __str__(self):
        return self.name


class Customer(Person):
    # choices : ('saved in db', 'how it shows on admin page')
    shirt_size = models.CharField(
        max_length=1, choices=[("S", "Small"), ("M", "Medium"), ("L", "Large")],
    )

    NATIONAL_CHOICES = (("US", "The United States"), ("KR", "South Korea"))
    nationality = models.CharField(max_length=2, choices=NATIONAL_CHOICES)


class Runner(Person):
    MEDAL_TYPE = models.TextChoices("MEDAL_TYPE", "GOLD SILVER BRONZE")
    medal = models.CharField(blank=True, choices=MEDAL_TYPE.choices, max_length=10)


class CollegeStudent(Person):
    year_in_school = models.CharField(
        max_length=2,
        choices=[
            ("FR", "Freshman"),
            ("SO", "Sophomore"),
            ("JR", "Junior"),
            ("SR", "Senior"),
            ("GR", "Graduate"),
        ],
    )


""" relationship """

# many-to-one relationship
class Musician(models.Model):
    instrument = models.CharField(max_length=100)


class Album(models.Model):
    artist = models.ForeignKey(Musician, on_delete=models.CASCADE)
    name = models.CharField(max_length=100)
    release_date = models.DateField()
    num_start = models.IntegerField()


# many-to-many relationship
class Topping(models.Model):
    name = models.CharField(max_length=50)


class Pizza(models.Model):
    toppings = models.ManyToManyField(Topping)
    """ Generally, ManyToManyField instances should go 
    in the object that’s going to be edited on a form. 
    In the above example, toppings is in Pizza 
    (rather than Topping having a pizzas ManyToManyField) 
    because it’s more natural to think about a pizza having
    toppings than a topping being on multiple pizzas. 
    The way it’s set up above, the Pizza form would let 
    users select the toppings.
    """


class Publication(models.Model):
    title = models.CharField(max_length=30)

    class Meta:
        ordering = ["title"]

    def __str__(self):
        return self.title


class Article(models.Model):
    headline = models.CharField(max_length=100)
    publications = models.ManyToManyField(Publication)

    class Meta:
        ordering = ["headline"]

    def __str__(self):
        return self.headline


class Group(models.Model):  # Extra fields on many-to-many relationships

    name = models.CharField(max_length=128)
    members = models.ManyToManyField(Person, through="Membership")

    def __str__(self):
        return self.name


class Membership(models.Model):
    person = models.ForeignKey(Person, on_delete=models.CASCADE)
    group = models.ForeignKey(Group, on_delete=models.CASCADE)
    date_joined = models.DateField()
    invite_reason = models.CharField(max_length=64)

    def __str__(self):
        return f"{str(self.person)}__{str(self.group)}"


# One-to-one relationship

"""
OneToOneField requires a positional argument: the class to which the model is related.

For example, if you were building a database of "places", you would build pretty standard
 stuff such as address, phone number, etc. in the database. Then, if you wanted to build 
 a database of restaurants on top of the places, instead of repeating yourself and 
 replicating those fields in the Restaurant model, you could make Restaurant have a 
 OneToOneField to Place (because a restaurant "is a" place; in fact, to handle this you'd 
 typically use inheritance, which involves an implicit one-to-one relation).
"""


class Place(models.Model):
    zipcode = models.CharField(max_length=30)


class Restaurant(models.Model):
    place = models.OneToOneField(Place, on_delete=models.CASCADE)


# Meta Options
"""
Model metadata is "anything that's not a field", such as ordering options (ordering), 
database table name (db_table), or human-readable singular and plural names 
(verbose_name and verbose_name_plural). 

None are required, and adding class Meta to a model is completely optional.
"""


class Ox(models.Model):
    horn_length = models.IntegerField()

    class Meta:
        ordering = ["horn_length"]
        verbose_name_plural = "oxen"


# Model attributes
""" object (default Manager)

The most important attribute of a model is the Manager. It's the interface through which
database query operations are provided to Django models and is used to retrieve the instances 
from the database. If no custom Manager is defined, the default name is objects. 

Managers are only accessible via model classes, not the model instances.
"""


# Model methods
"""
you can define a custom method
- This is a valuable technique for keeping business logic in one place -- the model.
"""


class PersonFL(models.Model):
    first_name = models.CharField(max_length=50)
    last_name = models.CharField(max_length=50)

    @property
    def full_name(self):
        return f"{self.first_name} {self.last_name}"


# Overriding predefined model methods
"""
In particular you'll often want to change the way save() and delete() work.

It's important to remember to call the superclass method 
"""


class Blog(models.Model):
    name = models.CharField(max_length=100)
    tag_line = models.TextField()

    def save(self, *args, **kwargs):
        # do_something()
        super().save(*args, **kwargs)  # Call the "real" save() method.
        # do_something_else()


# Model inheritance
"""
The base class should subclass: django.db.models.Model.

there are three styles of inheritance in Django
    1. abstract class
        the parent class to hold information that you 
        don't want to have to type out for each child model
    
    2. Multi-table inheritance
        subclasses have their own databases
    
    3. Proxy model
        only change python-level behavior,
        no change in fields
"""

# 1. abstract class
"""
the parent class to hold information that you 

abstract=True in the Meta class
    -> will not create any database table
"""


class CommonImfo(models.Model):
    """ abstract class
    no tables, no Manager
    """

    name = models.CharField(max_length=100)
    age = models.PositiveIntegerField()

    class Meta:
        abstract = True


class Student(CommonImfo):
    home_group = models.CharField(max_length=5)

    """ Meta inheritance
    if you need to re-define Meta class
        -> you can inherit parents' meta class 

    even if you don't inherit meta class, 
    django sets abstract=False automatically
    """

    class Meta(CommonImfo.Meta):
        db_table = "student_ifno"
        # abstract=True # explicitly set if you want a subclass to be a abstract class


# Be careful with related_name and related_query_name
"""
To work around this problem, when you are using related_name or related_query_name
in an abstract base class (only), part of the value should contain '%(app_label)s' 
and '%(class)s'.

if you forget to use it, Django will raise errors when you perform system checks 
(or run migrate).
"""
from django.db import models


class Ingredients(models.Model):
    pass


class Burger(models.Model):
    m2m = models.ManyToManyField(
        Ingredients,
        related_name="%(app_label)s_%(class)s_related",
        related_query_name="%(app_label)s_%(class)ss",
    )

    class Meta:
        abstract = True


class CheeseBurger(Burger):
    pass
    # reverse m2m field name: db_models_cheeseburger_related
    # reverse query name: db_models_cheeseburgers


# 2. Multi-table inheritance
"""
Each model corresponds to its own database table and can be queried and 
created individually. The inheritance relationship introduces links between 
the child model and each of its parents (via an automatically-created OneToOneField)
"""


class PlaceA(models.Model):
    name = models.CharField(max_length=50)
    address = models.CharField(max_length=80)


class RestaurantA(PlaceA):
    """
    All of the fields of Place will also be available in Restaurant, 
    although the data will reside in a different database table. 
    """

    # below is how automatically created one-to-one fields looks like
    place_ptr = models.OneToOneField(
        Place, on_delete=models.CASCADE, parent_link=True, primary_key=True,
    )

    serves_hot_dogs = models.BooleanField(default=False)
    serves_pizza = models.BooleanField(default=False)


# 3. Proxy models
"""
unlike multi-table inheritance, proxy model doesn't have it' own database.
    -> share database with parents

This is useful when only modifying a python-level methods 
"""


class PersonB(models.Model):
    first_name = models.CharField(max_length=30)
    last_name = models.CharField(max_length=30)


class MyPerson(PersonB):
    class Meta:
        proxy = True

    def do_something(self):
        # ...
        pass
