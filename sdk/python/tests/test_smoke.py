
from m0club import M0Client

def test_import():
    c = M0Client(base_url="http://localhost:8080")
    c.close()
