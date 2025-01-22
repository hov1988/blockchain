use std::u64;
use crate::blockchain::*;

#[derive(Debug)]
pub struct Transaction {
    sender_address: Vec<u8>,
    recipient_address: Vec<u8>,
    value: u64
}


impl Transaction {
    pub fn new(sender: Vec<u8>, receiver: Vec<u8>, value: u64) -> Transaction {
        Transaction{
            sender_address: sender,
            recipient_address: receiver,
            value
        }
    }
}

impl Transition<Transaction> for Transaction {
    fn serialisation(&self) -> Vec<u8> {
        let mut bin = Vec::<u8>::new();
        let len_sender = self.sender_address.len();
        bin.extend(len_sender.to_be_bytes().to_vec());
        bin.extend(&self.sender_address);

        let len_recipient = self.recipient_address.len();
        bin.extend(len_recipient.to_be_bytes().to_vec());
        bin.extend(&self.recipient_address);

        let len_value = self.value.to_be_bytes().len();
        bin.extend(len_value.to_be_bytes().to_vec());
        bin.extend(&self.value.to_be_bytes().to_vec());

        bin
    }

    fn deserialization(bytes: Vec<u8>) -> Transaction {
        let mut pos = 8;

        let len_sender = usize::from_be_bytes(bytes[pos..pos+8].iter().try_into().unwrap());
        let mut sender_address = Vec::<u8>::new();
        pos+=8;
        sender_address.extend_from_slice(&bytes[pos..pos+len_sender]);
        pos+=len_sender;

        let len_recipient = usize::from_be_bytes(bytes[pos..pos+8].iter().try_into().unwrap());
        pos+=8;
        let mut sender_recipient = Vec::<u8>::new();
        sender_recipient.extend_from_slice(&bytes[pos..pos+len_recipient]);
        pos+=len_recipient;

        let len_value = usize::from_be_bytes(bytes[pos..pos+8].iter().try_into().unwrap());
        let value = u64::from_be_bytes(bytes[pos..pos+len_value].iter().try_into().unwrap());
        pos+=8;

        Transaction {
            sender_address,
            recipient_address,
            value

        }
    }

}