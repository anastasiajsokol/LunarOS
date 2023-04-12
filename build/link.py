"""
    Link compiled binaries into image
        Current version 0.1 uses the Fat12 format
"""

def link_layout(layout: dict) -> bytes:
    return b''

if __name__ == "__main__":
    from sys import argv

    if len(argv) != 2:
        print("Linker expects a layout json file as an argument")
        print("For more information see docs")
    
    