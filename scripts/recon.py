import socket
import sys

def scan(domain):
    # En yaygın subdomainler
    subs = ["www", "mail", "api", "dev", "test", "admin", "blog", "vpn"]
    print(f"\033[1;34m[*] {domain} için keşif başlatıldı...\033[0m")
    
    for sub in subs:
        target = f"{sub}.{domain}"
        try:
            ip = socket.gethostbyname(target)
            print(f"\033[1;32m[+] Bulundu:\033[0m {target} -> {ip}")
        except socket.gaierror:
            pass

if __name__ == "__main__":
    if len(sys.argv) > 1:
        target_domain = sys.argv[1]
    else:
        target_domain = input("Hedef Domain (örn: istye.edu.tr): ")
    scan(target_domain)
