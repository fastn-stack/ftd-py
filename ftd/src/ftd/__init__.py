import json
import os
from typing import Optional, Callable

import ftd_sys


class Document:
    # noinspection PyShadowingBuiltins
    def __init__(
        self,
        id: str, *,
        root: Optional[str] = None,
        base_url: Optional[str] = None,
        handle_processor: Callable = None,
        handle_foreign_variable: Callable = None,
        handle_import: Callable = None,
        **data
    ):
        self.id = id
        if not root:
            try:
                root = os.environ["FPM_PACKAGE_ROOT"]
            except KeyError:
                pass
        self.root = root
        self.data = data
        self.base_url = base_url if base_url else "/"
        self.handle_processor = handle_processor
        self.handle_foreign_variable = handle_foreign_variable
        self.handle_import = handle_import

    def render(self, **data) -> str:
        all_data = self.data
        all_data.update(data)
        all_data = json.dumps(all_data)
        # noinspection PyUnresolvedReferences
        return interpret(
            self.id,
            root=self.root,
            base_url=self.base_url,
            handle_processor=self.handle_processor,
            handle_foreign_variable=self.handle_foreign_variable,
            handle_import=self.handle_import,
            data=all_data,
        )


# noinspection PyShadowingBuiltins
def parse(
    id: str, *,
    root: Optional[str] = None,
    base_url: Optional[str] = None,
    handle_processor: Callable = None,
    handle_foreign_variable: Callable = None,
    handle_import: Callable = None,
    **data
) -> Document:
    return Document(
        id,
        root=root,
        base_url=base_url,
        handle_processor=handle_processor,
        handle_foreign_variable=handle_foreign_variable,
        handle_import=handle_import,
        **data,
    )


# noinspection PyShadowingBuiltins
def render(
    id: str, *,
    root: Optional[str] = None,
    base_url: Optional[str] = None,
    handle_processor: Callable = None,
    handle_foreign_variable: Callable = None,
    handle_import: Callable = None,
    **data
) -> str:
    d = parse(
        id,
        root=root,
        base_url=base_url,
        handle_processor=handle_processor,
        handle_foreign_variable=handle_foreign_variable,
        handle_import=handle_import,
        **data,
    )
    return d.render()


def interpret(
    id: str, *,
    root: Optional[str] = None,
    base_url: Optional[str] = None,
    handle_processor: Callable = None,
    handle_foreign_variable: Callable = None,
    handle_import: Callable = None,
    data: Optional[str] = None,
):
    try:
        interpreter = ftd_sys.interpret(id, root, base_url, data)
        while True:
            print("in ftd interpreter-loop")
            state = interpreter.state_name()
            if state == "done":
                return interpreter.render()

            if state == "stuck_on_import":
                print("stuck_on_import")
                module = interpreter.get_module_to_import()
                source = interpreter.resolve_import(module)
                if not source:
                    if not handle_import:
                        raise Exception("can not import: %s" % module)
                    source = handle_import(module)

                # TODO: May need to take some different approach
                # if rust is sending empty string, python taking it None
                if source == "__import_resolved__":
                    source = ""

                interpreter.continue_after_import(module, source)
                print("stuck_on_import done")

            if state == "stuck_on_foreign_variable":
                print("stuck_on_foreign_variable")
                variable = interpreter.get_foreign_variable_to_resolve()
                print("variable: ", variable)
                value = interpreter.resolve_foreign_variable(variable, base_url)
                if not value:
                    if not handle_foreign_variable:
                        raise Exception(
                            "can not import foreign variable: %s" % variable)
                    value = handle_foreign_variable(variable)
                interpreter.continue_after_foreign_variable(variable, value)
                print("stuck_on_foreign_variable done")

            if state == "stuck_on_processor":
                print("stuck_on_processor")
                section = interpreter.get_processor_section()
                processor_value = interpreter.resolve_processor(section)

                """
                If value returned from fpm processors is None, we will call to
                python application processor 
                """
                if not processor_value:
                    if not handle_processor:
                        raise Exception(
                            "can not handle processor: %s" % section)
                    processor_value = handle_processor(section)
                interpreter.continue_after_processor(processor_value)
                print("stuck_on_processor done")

    except Exception as e:
        print("Exception in interpreter: ", e)
        return e


def file_content(root: str, path: str) -> (bytearray, str):
    return ftd_sys.get_file_content(root, path)
