use crate::blockchain::*;
use std::fmt;
use std::str;

#[derive(Debug)]
pub struct Transaction {
    pub sender_address: Vec<u8>,
    pub recipient_address: Vec<u8>,
    pub value: u64,
}

impl Transaction {
    pub fn new(sender: Vec<u8>, recipient: Vec<u8>, value: u64) -> Transaction {
        Transaction {
            sender_address: sender,
            recipient_address: recipient,
            value,
        }
    }
}

impl Serialization<Transaction> for Transaction {
    fn serialisation(&self) -> Vec<u8> {
        let mut bin = Vec::<u8>::new();
        let len_sender = self.sender_address.len();
        bin.extend(len_sender.to_be_bytes().to_vec());
        bin.extend(&self.sender_address);
        let len_recipient = self.recipient_address.len();
        bin.extend(len_recipient.to_be_bytes().to_vec());
        bin.extend(&self.recipient_address);
        bin.extend(self.value.to_be_bytes());
        bin
    }

    fn deserialization(bytes: Vec<u8>) -> Transaction {
        let mut pos = 0;
        let len_sender = usize::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let sender_address = bytes[pos..pos + len_sender].to_vec();
        pos += len_sender;

        let len_recipient = usize::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let recipient_address = bytes[pos..pos + len_recipient].to_vec();
        pos += len_recipient;

        let value = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());

        Transaction {
            sender_address,
            recipient_address,
            value,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sender = str::from_utf8(&self.sender_address).unwrap_or("<Invalid UTF-8>");
        let recipient = str::from_utf8(&self.recipient_address).unwrap_or("<Invalid UTF-8>");
        write!(
            f,
            "{}\nSender address: {}\nRecipient address: {}\nValue: {}\n{}",
            "-".repeat(40),
            sender,
            recipient,
            self.value,
            "-".repeat(40)
        )
    }
}