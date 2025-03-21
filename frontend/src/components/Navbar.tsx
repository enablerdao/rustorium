import {
  Box,
  Flex,
  HStack,
  IconButton,
  Button,
  Menu,
  MenuButton,
  MenuList,
  MenuItem,
  useColorMode,
  useColorModeValue,
  Text,
} from '@chakra-ui/react';
import { Link as RouterLink } from 'react-router-dom';
import { FiSun, FiMoon, FiMenu } from 'react-icons/fi';

export default function Navbar() {
  const { colorMode, toggleColorMode } = useColorMode();
  const bg = useColorModeValue('white', 'gray.800');

  return (
    <Box bg={bg} px={4} shadow="md">
      <Flex h={16} alignItems="center" justifyContent="space-between">
        <HStack spacing={8} alignItems="center">
          <Text fontSize="xl" fontWeight="bold" as={RouterLink} to="/">
            Rustorium Dashboard
          </Text>
          <HStack as="nav" spacing={4} display={{ base: 'none', md: 'flex' }}>
            <Button as={RouterLink} to="/" variant="ghost">
              Dashboard
            </Button>
            <Button as={RouterLink} to="/blocks" variant="ghost">
              Blocks
            </Button>
            <Button as={RouterLink} to="/transactions" variant="ghost">
              Transactions
            </Button>
            <Button as={RouterLink} to="/validators" variant="ghost">
              Validators
            </Button>
          </HStack>
        </HStack>

        <Flex alignItems="center">
          <IconButton
            aria-label="Toggle color mode"
            icon={colorMode === 'light' ? <FiMoon /> : <FiSun />}
            onClick={toggleColorMode}
            mr={4}
          />
          <Menu>
            <MenuButton
              as={IconButton}
              aria-label="Menu"
              icon={<FiMenu />}
              variant="outline"
              display={{ base: 'block', md: 'none' }}
            />
            <MenuList>
              <MenuItem as={RouterLink} to="/">
                Dashboard
              </MenuItem>
              <MenuItem as={RouterLink} to="/blocks">
                Blocks
              </MenuItem>
              <MenuItem as={RouterLink} to="/transactions">
                Transactions
              </MenuItem>
              <MenuItem as={RouterLink} to="/validators">
                Validators
              </MenuItem>
            </MenuList>
          </Menu>
        </Flex>
      </Flex>
    </Box>
  );
}
