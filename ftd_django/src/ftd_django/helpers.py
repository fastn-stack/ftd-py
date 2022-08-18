from django.core.exceptions import ImproperlyConfigured, SuspiciousFileOperation
from django.utils.module_loading import import_string

from typing import Optional, List
from os.path import abspath, dirname, join, normcase, sep


BACKEND_NAME = "ftd_django.TemplateBackend"


def _get_base(base: str, debug_base: Optional[str], debug: bool) -> str:
    # if in prod mode, use base
    if not debug:
        return base
    # if debug_base is not set, use "/" in debug mode
    if debug_base is None:
        return "/"
    # if debug_base is False use base
    if debug_base is False:
        return base
    # if debug_base is anything else, return debug_base
    if isinstance(debug_base, str):
        return debug_base
    raise ImproperlyConfigured(
        'OPTIONS["debug-base"] must be False or str, found: %s', debug_base
    )


def validate_settings() -> (str, str):
    from django.conf import settings

    t = None
    for t_ in settings.TEMPLATES:
        if t_["BACKEND"] != BACKEND_NAME:
            continue
        if t:
            raise ImproperlyConfigured('"%s" configured more than once' % BACKEND_NAME)
        t = t_

    if t.get("APP_DIRS"):
        raise ImproperlyConfigured("APP_DIRS is not supported, please set it to False")

    dlen = len(t["DIRS"])
    if dlen != 1:
        raise ImproperlyConfigured("DIRS must contain a single entry, found %s" % dlen)

    options = t.get("OPTIONS", {})
    initialize_processors(options.get("PROCESSORS", []))
    folder = t["DIRS"][0]

    if not options:
        # noinspection PyRedundantParentheses
        return ("/", folder)

    return (
        _get_base(options.get("base", "/"), options.get("debug-base"), settings.DEBUG),
        folder,
    )


# Source: https://github.com/django/django/blob/main/django/utils/_os.py#L9-L35
# Copied because it's a private module in django
def safe_join(base, *paths):
    """
    Join one or more path components to the base path component intelligently.
    Return a normalized, absolute version of the final path.

    Raise ValueError if the final path isn't located inside of the base path
    component.
    """
    final_path = abspath(join(base, *paths))
    base_path = abspath(base)
    # Ensure final_path starts with base_path (using normcase to ensure we
    # don't false-negative on case insensitive operating systems like Windows),
    # further, one of the following conditions must be true:
    #  a) The next character is the path separator (to prevent conditions like
    #     safe_join("/dir", "/../d"))
    #  b) The final path must be the same as the base path.
    #  c) The base path must be the most root path (meaning either "/" or "C:\\")
    if (
        not normcase(final_path).startswith(normcase(base_path + sep))
        and normcase(final_path) != normcase(base_path)
        and dirname(normcase(base_path)) != normcase(base_path)
    ):
        raise SuspiciousFileOperation(
            "The joined path ({}) is located outside of the base path "
            "component ({})".format(final_path, base_path)
        )
    return final_path


PROCESSORS = {}


def initialize_processors(processors: List[str]):
    for p in processors:
        p = import_string(p)
        PROCESSORS[p.processor_name] = p


def processor(request):
    def p(doc_id, section, interpreter):

        name = section.header_string(doc_id, "$processor$")
        if name in PROCESSORS:
            return PROCESSORS[name](request, doc_id, section, interpreter)

        raise Exception(f"{name}: unknown processor")
    return p
