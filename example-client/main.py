"""Example gRPC client for simple-jab-wrapper."""

import grpc
import jab_wrapper_pb2
import jab_wrapper_pb2_grpc


def make_locator(role=None, name=None, description=None, states=None, strict=False):
    """Helper to create an ElementLocator."""
    locator = jab_wrapper_pb2.ElementLocator(strict=strict)
    if role:
        locator.role = role
    if name:
        locator.name = name
    if description:
        locator.description = description
    if states:
        locator.states.extend(states if isinstance(states, list) else [states])
    return locator


def main():
    channel = grpc.insecure_channel("127.0.0.1:9250")
    stub = jab_wrapper_pb2_grpc.JabServiceStub(channel)

    # Get version info
    print("Getting version info...")
    response = stub.GetVersionInfo(jab_wrapper_pb2.GetVersionInfoRequest())
    print(f"Version: {response.version}")

    # List windows
    print("Getting version info...")
    response = stub.ListWindows(jab_wrapper_pb2.ListWindowsRequest())
    print(f"Version: {response.version}")

    # Example: Select a window (you'll need a valid HWND)
    # response = stub.SelectWindow(jab_wrapper_pb2.SelectWindowRequest(hwnd=12345))
    # print(f"SelectWindow success: {response.success}, error: {response.error}")

    # Example: Find elements with structured locator
    # locator = make_locator(role="button", name="OK")
    # response = stub.FindElements(jab_wrapper_pb2.FindElementsRequest(
    #     locator=locator, max_depth=50
    # ))
    # print(f"Found {len(response.elements)} elements")
    # for elem in response.elements:
    #     print(f"  - {elem.role}: {elem.name} at ({elem.x}, {elem.y})")


if __name__ == "__main__":
    main()
