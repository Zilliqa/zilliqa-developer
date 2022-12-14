# Message Dispatch and Processing

In `src/cmd/main.cpp`, we assign `Zilliqa::Dispatch` as the dispatcher inside `P2PComm::StartMessagePump`.
Every message that is read from a socket by P2PComm then gets sent to `Zilliqa::Dispatch`.

When Zilliqa starts to process a message, it will call `Zilliqa::ProcessMessage`.
The first byte of any message defines the **message type**.

> **Note:** The “first byte” here refers to the payload part of a socket message.
> At the P2PComm level, each socket message consists of a predefined header plus the payload.

Depending on the type, `Zilliqa::ProcessMessage` will forward the message to the appropriate handler for it.
The list of message types can be found in `enum MessageType` inside `src/common/Messages.h`.

Any class that inherits from `Executable` will be a message handler.
For example, type `0x01` means `DIRECTORY`, and this message will be handled by `libDirectoryService`.
If you go into `libDirectoryService`, you will find a function `DirectoryService::Execute`.

All classes that inherit from `Executable` will first check the second byte in the message, which defines the **instruction type**.
The list of instruction types can be found in `src/common/Messages.h`.

From there, `Execute()` will further forward the message to a private function inside the class, and these functions are all named `ProcessXXX`.

## Message Queues and Thread Pools

Incoming and outgoing message queues are maintained between `P2PComm` and the rest of the Zilliqa core. This helps provide some ordering in the processing of messages, and it also adds some control over the number of messages that can be buffered. Once ready for processing, messages enter a thread pool, which helps control the number of messages that can be processed concurrently.

After an incoming message is read from a socket, it is first inserted into `Zilliqa::m_msgQueue`, whose maximum size is controlled by `MSGQUEUE_SIZE`. When the queue reaches full capacity, any further incoming messages are dropped. A dedicated thread launched during startup manages dequeueing of messages and sending them to `Zilliqa::m_queuePool`, a thread pool limited by `MAXMESSAGE`. Once assigned to a thread, the message gets dispatched according to the earlier section.

Equivalently, before an outgoing message is written out to a socket, it is first inserted into `P2PComm::m_sendQueue`, whose maximum size is controlled by `SENDQUEUE_SIZE`. Any further outgoing messages are also dropped once the queue is full. A dedicated thread launched during startup also manages dequeueing of messages and sending them to `Zilliqa::m_SendPool`, which is also limited by `MAXMESSAGE`. One assigned to a thread, the message gets sent out according to the `P2PComm::SendJob` settings for the message.
