#!/usr/bin/env python3

"""
test-python - Test Python extension
Author: testuser
"""

import os
import sys
import json
from datetime import datetime

class Colors:
    RED = '\033[0;31m'
    GREEN = '\033[0;32m'
    YELLOW = '\033[1;33m'
    BLUE = '\033[0;34m'
    NC = '\033[0m'

def print_info(message: str) -> None:
    print(f"{Colors.BLUE}ℹ️  {message}{Colors.NC}")

def print_success(message: str) -> None:
    print(f"{Colors.GREEN}✅ {message}{Colors.NC}")

def print_warning(message: str) -> None:
    print(f"{Colors.YELLOW}⚠️  {message}{Colors.NC}")

def print_error(message: str) -> None:
    print(f"{Colors.RED}❌ {message}{Colors.NC}")

def get_pm_context():
    """Get PM environment context"""
    return {
        'current_project': os.environ.get('PM_CURRENT_PROJECT'),
        'config_path': os.environ.get('PM_CONFIG_PATH'),
        'pm_version': os.environ.get('PM_VERSION')
    }

def run_command():
    """Run the Python extension functionality"""
    print_success("test-python Extension - Deploy Command")
    print_info("Executing Python extension functionality...")
    
    context = get_pm_context()
    if context['current_project']:
        print_info(f"Current PM project: {context['current_project']}")
    
    if context['config_path']:
        print_info(f"PM config: {context['config_path']}")
    
    print("🐍 Python extension is running successfully!")
    print("📦 Working with Python packages and virtual environments")
    print("🚀 Rich ecosystem and powerful libraries available")
    print("⚡ High-level programming with great readability")
    
    # Simulate some Python-specific functionality
    print_info("Checking Python environment...")
    print_info(f"Python version: {sys.version.split()[0]}")
    print_info(f"Python executable: {sys.executable}")

def check_command():
    """Check Python extension health"""
    print_success("test-python Extension - Check Command")
    print_info("Extension Type: Python")
    print_info("Version: 1.0.0")
    print_info("Status: Active and ready")
    
    # Check Python version and modules
    print_info(f"Python: {sys.version}")
    print_info(f"Platform: {sys.platform}")
    
    # Check if in PM context
    context = get_pm_context()
    if context['current_project']:
        print_success("Running in PM project context")
        try:
            project_data = json.loads(context['current_project'])
            print_info(f"Project: {project_data.get('name', 'Unknown')}")
            print_info(f"Path: {project_data.get('path', 'Unknown')}")
        except:
            print_info(f"Project: {context['current_project']}")
    else:
        print_warning("Not running in PM project context")
    
    # Check available modules
    try:
        import json
        import datetime
        import os
        print_success("Core Python modules available")
    except ImportError as e:
        print_error(f"Module import error: {e}")

def config_command():
    """Configure Python extension"""
    print_success("test-python Extension - Config Command")
    print_info("Python extension configuration")
    
    config = {
        "extension": "test-python",
        "version": "1.0.0",
        "python_version": sys.version.split()[0],
        "timestamp": datetime.now().isoformat(),
        "environment": dict(os.environ)
    }
    
    print_info("Current configuration:")
    print(f"  Extension: {config['extension']}")
    print(f"  Version: {config['version']}")
    print(f"  Python: {config['python_version']}")
    print(f"  Timestamp: {config['timestamp']}")
    
    context = get_pm_context()
    if context['current_project']:
        print(f"  PM Project: {context['current_project']}")
    if context['config_path']:
        print(f"  PM Config: {context['config_path']}")

def show_help():
    """Show help information"""
    print("Usage: pm test-python [COMMAND]")
    print("")
    print("Available Commands:")
    print("  deploy     Deploy using Python extension")
    print("  check      Check Python extension health") 
    print("  config     Configure Python extension")
    print("")
    print("PM Environment Variables:")
    print("  PM_CURRENT_PROJECT - Current project context")
    print("  PM_CONFIG_PATH     - PM configuration path")
    print("  PM_VERSION         - PM version")
    print("")
    print("Extension: test-python")
    print("Description: Test Python extension for local installation testing")
    print("Author: testuser")
    print("Homepage: https://github.com/testuser/test-python")

def main():
    """Main entry point"""
    if len(sys.argv) < 2:
        show_help()
        return
    
    command = sys.argv[1]
    
    if command == "deploy":
        run_command()
    elif command == "check":
        check_command()
    elif command == "config":
        config_command()
    else:
        show_help()

if __name__ == "__main__":
    main()