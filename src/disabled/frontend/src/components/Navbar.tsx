import React from 'react';
import { Box, Flex, Link, Spacer } from '@chakra-ui/react';
import { Link as RouterLink } from 'react-router-dom';

function Navbar() {
  return (
    <Box bg="blue.500" px={4} py={3}>
      <Flex maxW="container.xl" mx="auto" alignItems="center">
        <Link
          as={RouterLink}
          to="/"
          fontSize="xl"
          fontWeight="bold"
          color="white"
          _hover={{ textDecoration: 'none' }}
        >
          Rustorium
        </Link>
        <Spacer />
        <Flex gap={4}>
          <Link
            as={RouterLink}
            to="/"
            color="white"
            _hover={{ textDecoration: 'none', color: 'blue.100' }}
          >
            Wallet
          </Link>
        </Flex>
      </Flex>
    </Box>
  );
}

export default Navbar;