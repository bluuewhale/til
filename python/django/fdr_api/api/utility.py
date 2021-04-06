from collections import defaultdict


def merge_dict(*dicts):

    result = defaultdict(lambda: [])

    for dict_ in dicts:
        keys = dict_.keys()

        for key in keys:
            result[key] += list(dict_.get(key))

    return result


def dl_to_ld(dl):
    """ convert dictonary of lists to list of dictionaries"""

    return [dict(zip(dl, t)) for t in zip(*dl.values())]


def ld_to_dl(ld):
    """ convert list of dictionaries to dictonary of lists"""

    return {k: [dic[k] for dic in ld] for k in ld[0]}
