import os
import platform
import requests
import logging
from datetime import datetime

# ---------------------------
# LOGGING CONFIGURATION
# ---------------------------
logging.basicConfig(
    filename="aimd_agent.log",
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s"
)

class AIMD:
    def __init__(self, exp_time, port, heartbeat_interval=10):
        self.exp_time = exp_time
        self.pid = os.getpid()
        self.port = port
        self.heartbeat_interval = heartbeat_interval

        self.url_register = f"http://localhost:{port}/register"
        self.url_ping = f"http://localhost:{port}/ping"
    
    # -------------------------------------------------------
    # REGISTER AGENT — includes timestamp + platform metadata
    # -------------------------------------------------------
    def register_agent(self):
        # Auto-generate current timestamp in ISO format
        timestamp = datetime.now().isoformat()

        payload = {
            "platform": {
                "os": platform.system().lower(),
                "arch": platform.machine()
            },
            "process": {
                "pid": self.pid
            },
            "action": "Register",
            "timestamp": timestamp,
            "expires_in": self.exp_time,
            "port": self.port
        }

        logging.info("Attempting agent registration...")

        # Check if server returned an HTTP error
        try:
            resp = requests.post(self.url_register, json=payload)
            resp.raise_for_status()
        except Exception as e:
            logging.error(
                f"Registration failed: HTTP {getattr(resp, 'status_code', 'N/A')} - {e}"
            )
            raise RuntimeError("Registration failed — see log for details")

        # Return parsed JSON
        data = resp.json()
        logging.info(f"Registration successful: UID={data.get('uid')} PID={data.get('pid')}")

        return (
            data["uid"], 
            data["token"], 
            data["os"], 
            data["arch"], 
            data["timestamp"], 
            data["expires_at"], 
            data["pid"],
            data["port"]
        )