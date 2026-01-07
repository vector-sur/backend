#!/bin/bash

# Colores para output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# URL base del servidor
BASE_URL="http://localhost:3000"

echo -e "${BLUE}=== Test de API de Autenticación ===${NC}\n"

# 1. REGISTRAR UN NUEVO USUARIO
echo -e "${YELLOW}1. Registrando nuevo usuario...${NC}"
REGISTER_RESPONSE=$(curl -s -X POST "${BASE_URL}/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "mateo",
    "name": "Mateo",
    "lastname": "Lafalce",
    "phone": 1234567890,
    "email": "mateo@example.com",
    "password": "mateo"
  }')

echo -e "${GREEN}Response:${NC}"
echo "$REGISTER_RESPONSE" | jq '.'

# Extraer el token del registro
TOKEN=$(echo "$REGISTER_RESPONSE" | jq -r '.token')
echo -e "\n${GREEN}Token obtenido del registro:${NC} $TOKEN\n"

# 2. LOGIN CON EL USUARIO CREADO
echo -e "${YELLOW}2. Haciendo login con el usuario...${NC}"
LOGIN_RESPONSE=$(curl -s -X POST "${BASE_URL}/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "mateo",
    "password": "mateo"
  }')

echo -e "${GREEN}Response:${NC}"
echo "$LOGIN_RESPONSE" | jq '.'

# Extraer el token del login
TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.token')
echo -e "\n${GREEN}Token obtenido del login:${NC} $TOKEN\n"

# 3. ACCEDER A RUTA PROTEGIDA
echo -e "${YELLOW}3. Accediendo a ruta protegida con el token...${NC}"
PROTECTED_RESPONSE=$(curl -s -X GET "${BASE_URL}/protected" \
  -H "Authorization: Bearer $TOKEN")

echo -e "${GREEN}Response:${NC}"
echo "$PROTECTED_RESPONSE" | jq '.'

# 4. INTENTAR ACCEDER SIN TOKEN (debería fallar)
echo -e "\n${YELLOW}4. Intentando acceder sin token (debería fallar)...${NC}"
NO_TOKEN_RESPONSE=$(curl -s -X GET "${BASE_URL}/protected" \
  -w "\nHTTP Status: %{http_code}")

echo -e "${GREEN}Response:${NC}"
echo "$NO_TOKEN_RESPONSE"

echo -e "\n${BLUE}=== Test completado ===${NC}"
