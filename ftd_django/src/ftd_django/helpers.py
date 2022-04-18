from django.core.exceptions import ImproperlyConfigured

from typing import Optional


BACKEND_NAME = "ftd_django.TemplateBackend"


def get_base(base: str, debug_base: Optional[str], debug: bool) -> str:
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
    folder = t["DIRS"][0]

    if not options:
        # noinspection PyRedundantParentheses
        return ("/", folder)

    return (
        get_base(options.get("base", "/"), options.get("debug-base"), settings.DEBUG),
        folder,
    )
