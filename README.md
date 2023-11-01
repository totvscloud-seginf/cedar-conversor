# CedarPy Conversor
![CI (main)](https://github.com/totvscloud-seginf/cedarpy-conversor/actions/workflows/CI.yml/badge.svg?branch=main)
&nbsp;[![PyPI version](https://badge.fury.io/py/cedarpy-conversor.svg)](https://badge.fury.io/py/cedarpy-conversor)

`cedarpy-conversor` helps you use the (Rust) [Cedar Policy](https://github.com/cedar-policy/cedar/tree/main) library from Python. You can use `cedarpy-conversor` to:
* convert cedar policies to JSON representation
* convert JSON representation to cedar policies

`cedarpy-conversor` packages are availble for the following platforms:
<table>
<thead><tr><th>Operating System</th><th>Processor Architectures</th></tr></thead>
<tbody>
    <tr><td>Linux</td><td>x86_64, aarch64</td></tr>
    <tr><td>Mac</td><td>x86_64, aarch64</td></tr>
    <tr><td>Windows</td><td>x86_64</td></tr>
</tbody>
</table>

Note: This project is _not_ officially supported by AWS or the Cedar Policy team.

## Using the library
Releases of [`cedarpy-conversor`](https://pypi.org/project/cedarpy-conversor/) are available on PyPi.  You can install the latest release with:
```shell
pip install cedarpy-conversor
```

(See the Developing section for how to use artifacts you've built locally.)

### Convert cedar policies to JSON representation
Now you can use the library to convert cedar policies to JSON representation:
```python
import json
from cedarpy_conversor import convert_cedar_policies_to_json

policies: str = """
    permit(
        principal == User::"12UA45",
        action == Action::"view",
        resource in Folder::"abc"
    ) when {
        (context["tls_version"]) == "1.3"
    };
"""
json_representation = json.loads(convert_cedar_policies_to_json(policies))
json_representation: str = json.dumps(json_representation, indent=2)

expected_json_representation: str = """
{
  "effect": "permit",
  "principal": {
    "op": "==",
    "entity": {
      "type": "User",
      "id": "12UA45"
    }
  },
  "action": {
    "op": "==",
    "entity": {
      "type": "Action",
      "id": "view"
    }
  },
  "resource": {
    "op": "in",
    "entity": {
      "type": "Folder",
      "id": "abc"
    }
  },
  "conditions": [
    {
      "kind": "when",
      "body": {
        "==": {
          "left": {
            ".": {
              "left": {
                "Var": "context"
              },
              "attr": "tls_version"
            }
          },
          "right": {
            "Value": "1.3"
          }
        }
      }
    }
  ]
}
"""

# so you can assert that the json representation is correct
assert json_representation.strip() == expected_json_representation.strip()
```

### Convert JSON representation to cedar policies
You can also use the library to convert JSON representation to cedar policies:
```python
import json
from cedarpy_conversor import convert_json_to_cedar_policies

expected_policies: str = """
permit(
  principal == User::"12UA45",
  action == Action::"view",
  resource in Folder::"abc"
) when {
  (context["tls_version"]) == "1.3"
};
"""


json_representation: str = """
{
  "effect": "permit",
  "principal": {
    "op": "==",
    "entity": {
      "type": "User",
      "id": "12UA45"
    }
  },
  "action": {
    "op": "==",
    "entity": {
      "type": "Action",
      "id": "view"
    }
  },
  "resource": {
    "op": "in",
    "entity": {
      "type": "Folder",
      "id": "abc"
    }
  },
  "conditions": [
    {
      "kind": "when",
      "body": {
        "==": {
          "left": {
            ".": {
              "left": {
                "Var": "context"
              },
              "attr": "tls_version"
            }
          },
          "right": {
            "Value": "1.3"
          }
        }
      }
    }
  ]
}
"""

policies: str = convert_json_to_cedar_policies(json_representation)

# so you can assert that the json representation is correct
assert policies.strip() == expected_policies.strip()
```

## Developing


You'll need a few things to get started:

* Python +3.9
* Rust and `cargo`

This project is built on the [PyO3](https://docs.rs/pyo3/latest/pyo3/index.html) and [maturin](https://www.maturin.rs/index.html) projects.  These projects are designed to enable Python to use Rust code and vice versa.

The most common development commands are in the `Makefile`

### Create virtual env

First create a Python virtual environment for this project with:
`make venv-dev`

In addition to creating a dedicated virtual environment, this will install `cedarpy-conversor`'s dependencies.

If this works you should be able to run the following command:
``` shell
maturin --help
```

## Build and run `cedarpy-conversor` tests

Ensure the `cedarpy-conversor` virtual environment is active by sourcing it in your shell:

```shell
source venv-dev/bin/activate
```

Now run:
```shell
make quick
```

The `make quick` command will build the Rust source code with `maturin` and run the project's tests with `pytest`.

If all goes well, you should see output like:
```shell
(venv-dev) totvs:cedarpy-conversor totvs$ make quick
Performing quick build
set -e ;\
	maturin develop ;\
	pytest
📦 Including license file "/path/to/cedarpy-conversor/LICENSE"
🔗 Found pyo3 bindings
🐍 Found CPython 3.9 at /path/to/cedarpy-conversor/venv-dev/bin/python
📡 Using build options features from pyproject.toml
Ignoring maturin: markers 'extra == "dev"' don't match your environment
Ignoring pip-tools: markers 'extra == "dev"' don't match your environment
Ignoring pytest: markers 'extra == "dev"' don't match your environment
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
   Compiling cedarpy_conversor v0.1.0 (/path/to/cedarpy-conversor)
    Finished dev [unoptimized + debuginfo] target(s) in 3.06s
📦 Built wheel for CPython 3.9 to /var/folders/k2/tnw8n1c54tv8nt4557pfx3440000gp/T/.tmpO6aj6c/cedarpy-0.1.0-cp39-cp39-macosx_11_0_arm64.whl
🛠 Installed cedarpy-0.1.0
================================================================================================ test session starts ================================================================================================
platform darwin -- Python 3.9.12, pytest-7.4.0, pluggy-1.2.0
rootdir: /path/to/cedarpy-conversor
configfile: pyproject.toml
testpaths: tests/unit
collected 4 items

tests/unit/test_convert_json_to_policy.py::ConvertJsonToPolicyTestCase::test_policy_json_to_cedar PASSED                                                                               [ 25%]
tests/unit/test_convert_policy_to_json.py::ConvertPolicyToJsonTestCase::test_policy_json_to_cedar PASSED                                                                               [ 50%] 
tests/unit/test_import_module.py::ImportModuleTestCase::test_cedarpy_conversor_module_imports PASSED                                                                                             [ 75%] 
tests/unit/test_import_module.py::InvokeModuleTestFunctionTestCase::test_invoke_echo PASSED                                                                                            [100%] 

================================================================================================ 4 passed in 0.32s =================================================================================================
```

### Using locally-built artifacts

If you used `make quick` above, then a development build of the `cedarpy-conversor` module will already be installed in the virtual environment. 

If you want to use your local `cedarpy-conversor` changes in another Python environment, you'll need to build a release with:

```shell
make release
```

The release process will build a wheel and output it into `target/wheels/`

Now you can install that file with pip, e.g.:
```shell
pip install --force-reinstall /path/to/cedarpy-conversor/target/wheels/cedarpy_*.whl
```


## Contributing

This project is in its early stages and contributions are welcome. Please check the project's GitHub [issues](https://github.com/totvscloud-seginf/cedarpy-conversor/issues) for work we've already identified.

Some ways to contribute are:
* Use the project and report experience and issues
* Document usage and limitations
* Enhance the library with additional functionality you need