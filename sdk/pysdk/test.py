import sys
import os
from aimd import AIMD

def main():
    req = AIMD(3000, 8080)
    if len(sys.argv) > 1:
        arg = sys.argv[1]
        if arg == "TEST":
            assert req.port == 8080, "Port did not update correctly!"
            assert req.exp_time == 3000, "Expiry time did not update correctly!"
            assert req.pid == os.getpid()
            print("[✔] Assertions passed")
        else:
            print("[-] Invalid argument")

        return

    uid, token, system, arch, timestamp, exp, pid, port = req.register_agent()

    print("UID:", uid)
    print("Token:", token)
    print("OS:", system)
    print("Arch:", arch)
    print("Timestamp:", timestamp)
    print("Expires:", exp)
    print("Process ID:", pid)
    print("Port:", port)

if __name__ == "__main__":
    main()