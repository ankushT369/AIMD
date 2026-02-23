import os
import requests

class AIMD:
    def __init__(self, exp_time, port):
        self.exp_time = exp_time
        self.pid = os.getpid()
        self.port = port
        self.url = f"http://localhost:{port}/register"

    def register_agent(self):
        payload = {
            "action": "Register",
            "expires_in": self.exp_time,
            "pid": self.pid
        }

        resp = requests.post(self.url, json=payload)

        # Check if server returned an HTTP error
        try:
            resp.raise_for_status()
        except Exception as e:
            raise RuntimeError(f"Server error: {resp.status_code}, Body: {resp.text}") from e

        # Return parsed JSON
        data = resp.json()
        return data["uid"], data["token"], data["expires_at"]