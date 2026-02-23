from aimd import AIMD

req = AIMD(3000, 8080)
uid, token, exp = req.register_agent()

print("UID:", uid)
print("Token:", token)
print("Expires:", exp)