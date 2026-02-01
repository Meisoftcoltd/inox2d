import inox2d

def test_context_creation():
    ctx = inox2d.Context()
    assert ctx is not None

def test_aliases():
    assert inox2d.Context == inox2d.InoxContext
    assert inox2d.Renderer == inox2d.InoxContext

def test_spec_version():
    assert isinstance(inox2d.INOCHI2D_SPEC_VERSION, str)
