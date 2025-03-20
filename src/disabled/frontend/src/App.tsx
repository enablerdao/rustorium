import React from 'react';
import { ChakraProvider, Box, VStack, Heading } from '@chakra-ui/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Wallet from './components/Wallet';
import Navbar from './components/Navbar';

const queryClient = new QueryClient();

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ChakraProvider>
        <Router>
          <Box minH="100vh" bg="gray.50">
            <Navbar />
            <VStack spacing={8} p={8}>
              <Heading>Rustorium Wallet</Heading>
              <Routes>
                <Route path="/" element={<Wallet />} />
              </Routes>
            </VStack>
          </Box>
        </Router>
      </ChakraProvider>
    </QueryClientProvider>
  );
}

export default App;