#!/usr/bin/env python3
"""
Генератор лицензионных ключей для NeuroObsessia.
"""

import hashlib
import random
import string
import sys

MASTER_KEY = "NEUROOBSESSIA-2026-KEY-PASS"
SECRET = "NeuroObsessia#2026#GlobalLicenseSecret"

def generate_key(prefix="NEURO") -> str:
    """Генерирует валидный ключ на 24 символа."""
    chars = string.ascii_uppercase + string.digits
    # Префикс/случайная соль до 12 символов
    needed = 12 - len(prefix)
    if needed > 0:
        nonce = prefix + "".join(random.choices(chars, k=needed))
    else:
        nonce = prefix[:12]
    
    # Хэш-подпись (12 символов)
    sig = hashlib.sha256((nonce + SECRET).encode("utf-8")).hexdigest().upper()[:12]
    raw = nonce + sig
    
    # Форматирование по 4 символа (XXXX-XXXX-XXXX-XXXX-XXXX-XXXX)
    return f"{raw[:4]}-{raw[4:8]}-{raw[8:12]}-{raw[12:16]}-{raw[16:20]}-{raw[20:24]}"

def main():
    count = 5
    if len(sys.argv) > 1:
        try:
            count = int(sys.argv[1])
        except ValueError:
            count = 5
            
    print("=" * 60)
    print("  NeuroObsessia Key Generator")
    print("=" * 60)
    print(f"\n[!] Универсальный мастер-ключ (работает ВСЕГДА):")
    print(f"    {MASTER_KEY}\n")
    print(f"[*] Сгенерированные уникальные ключи ({count} шт.):")
    for i in range(1, count + 1):
        print(f"    {i}. {generate_key()}")
    print("\n" + "=" * 60)

if __name__ == "__main__":
    main()
