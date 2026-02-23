import sys
from aimd import AIMD

req = AIMD(3000, 8080)
if len(sys.argv) > 1:
    assert req.port == 8080, "Port did not update correctly!"
    assert req.exp_time == 3000, "Expiry time did not update correctly!"
    print("Assertions passed ✔")

else:
    uid, token, exp = req.register_agent()

    print("UID:", uid)
    print("Token:", token)
    print("Expires:", exp)