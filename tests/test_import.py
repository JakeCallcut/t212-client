import t212client

def test_import_and_version():
    assert isinstance(t212client.__version__, str)
    assert t212client.__version__