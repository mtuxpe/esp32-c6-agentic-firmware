#!/usr/bin/env python3
"""
Serial port monitor for ESP32-C6 development.
Reads from serial port and prints to stdout with optional logging.

Usage:
    python3 monitor.py --port /dev/ttyUSB0 --baud 115200
    python3 monitor.py --port /dev/ttyACM0
"""

import serial
import sys
import argparse
from datetime import datetime


def main():
    parser = argparse.ArgumentParser(
        description='Serial port monitor for ESP32-C6',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 monitor.py --port /dev/ttyUSB0
  python3 monitor.py --port /dev/ttyACM0 --baud 115200
        """
    )
    parser.add_argument('--port', default='/dev/ttyUSB0', help='Serial port (default: /dev/ttyUSB0)')
    parser.add_argument('--baud', type=int, default=115200, help='Baud rate (default: 115200)')
    parser.add_argument('--log', help='Optional log file to write output')
    args = parser.parse_args()

    log_file = None
    try:
        # Open serial port
        ser = serial.Serial(args.port, args.baud, timeout=1)
        print(f"✓ Connected to {args.port} at {args.baud} baud", file=sys.stderr)
        print("=" * 70, file=sys.stderr)

        # Open optional log file
        if args.log:
            log_file = open(args.log, 'w')
            print(f"✓ Logging to {args.log}", file=sys.stderr)

        # Read and display output
        while True:
            if ser.in_waiting:
                line = ser.readline().decode('utf-8', errors='ignore').rstrip()
                if line:
                    # Print to stdout
                    print(line)
                    sys.stdout.flush()

                    # Write to log if specified
                    if log_file:
                        log_file.write(line + '\n')
                        log_file.flush()

    except KeyboardInterrupt:
        print("\n" + "=" * 70, file=sys.stderr)
        print("✓ Monitor stopped by user", file=sys.stderr)

    except FileNotFoundError:
        print(f"✗ Serial port {args.port} not found", file=sys.stderr)
        print("  Check connection and try again", file=sys.stderr)
        sys.exit(1)

    except Exception as e:
        print(f"✗ Error: {e}", file=sys.stderr)
        sys.exit(1)

    finally:
        if log_file:
            log_file.close()
        if 'ser' in locals() and ser.is_open:
            ser.close()


if __name__ == '__main__':
    main()
