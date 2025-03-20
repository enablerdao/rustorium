import React, { useState } from 'react';
import {
  Box,
  Button,
  VStack,
  Text,
  Input,
  useToast,
  Table,
  Thead,
  Tbody,
  Tr,
  Th,
  Td,
} from '@chakra-ui/react';
import { useMutation, useQuery } from '@tanstack/react-query';
import axios from 'axios';

const API_URL = 'http://localhost:8001';

interface WalletInfo {
  address: string;
  balance: number;
  token_balances: Array<{
    token_id: string;
    symbol: string;
    balance: number;
  }>;
}

function Wallet() {
  const toast = useToast();
  const [address, setAddress] = useState('');
  const [toAddress, setToAddress] = useState('');
  const [amount, setAmount] = useState('');

  // ウォレットを作成
  const createWallet = useMutation({
    mutationFn: async () => {
      const response = await axios.post(`${API_URL}/wallets`, {
        name: 'My Wallet',
      });
      return response.data;
    },
    onSuccess: (data) => {
      setAddress(data.address);
      toast({
        title: 'Wallet created!',
        description: `Address: ${data.address}\nInitial balance: ${data.initial_balance}`,
        status: 'success',
        duration: 5000,
        isClosable: true,
      });
    },
  });

  // ウォレット情報を取得
  const { data: walletInfo } = useQuery<WalletInfo>({
    queryKey: ['wallet', address],
    queryFn: async () => {
      if (!address) return null;
      const response = await axios.get(`${API_URL}/wallets/${address}`);
      return response.data;
    },
    enabled: !!address,
  });

  // トークンを送金
  const transfer = useMutation({
    mutationFn: async () => {
      const response = await axios.post(`${API_URL}/wallets/${address}/transfer`, {
        to: toAddress,
        amount: parseInt(amount),
      });
      return response.data;
    },
    onSuccess: (data) => {
      toast({
        title: 'Transfer successful!',
        description: `Transaction ID: ${data.transaction_id}\nNew balance: ${data.new_balance}`,
        status: 'success',
        duration: 5000,
        isClosable: true,
      });
      setToAddress('');
      setAmount('');
    },
  });

  return (
    <VStack spacing={6} w="full" maxW="container.md">
      <Box w="full">
        <Button
          colorScheme="blue"
          onClick={() => createWallet.mutate()}
          isLoading={createWallet.isLoading}
          w="full"
        >
          Create Wallet
        </Button>
      </Box>

      {address && (
        <Box w="full" p={6} bg="white" rounded="lg" shadow="md">
          <Text fontSize="lg" fontWeight="bold" mb={4}>
            Your Wallet
          </Text>
          <Text>Address: {address}</Text>
          <Text>Balance: {walletInfo?.balance ?? 0}</Text>

          <Box mt={6}>
            <Text fontSize="lg" fontWeight="bold" mb={4}>
              Token Balances
            </Text>
            <Table variant="simple">
              <Thead>
                <Tr>
                  <Th>Token</Th>
                  <Th>Symbol</Th>
                  <Th isNumeric>Balance</Th>
                </Tr>
              </Thead>
              <Tbody>
                {walletInfo?.token_balances.map((token) => (
                  <Tr key={token.token_id}>
                    <Td>{token.token_id}</Td>
                    <Td>{token.symbol}</Td>
                    <Td isNumeric>{token.balance}</Td>
                  </Tr>
                ))}
              </Tbody>
            </Table>
          </Box>

          <Box mt={6}>
            <Text fontSize="lg" fontWeight="bold" mb={4}>
              Transfer Tokens
            </Text>
            <VStack spacing={4}>
              <Input
                placeholder="To Address"
                value={toAddress}
                onChange={(e) => setToAddress(e.target.value)}
              />
              <Input
                placeholder="Amount"
                type="number"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
              />
              <Button
                colorScheme="green"
                onClick={() => transfer.mutate()}
                isLoading={transfer.isLoading}
                w="full"
              >
                Send
              </Button>
            </VStack>
          </Box>
        </Box>
      )}
    </VStack>
  );
}

export default Wallet;