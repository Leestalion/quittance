-- Create a test user for development
-- Password: "password123" (hashed with argon2)
INSERT INTO users (
    id,
    email,
    password_hash,
    name,
    address,
    phone
) VALUES (
    '00000000-0000-0000-0000-000000000000'::uuid,
    'test@example.com',
    '$argon2id$v=19$m=19456,t=2,p=1$VGVzdFNhbHQxMjM0NTY3OA$8K0wVJ6Jx9v8XZ9YqZ4Z5Z6Z7Z8Z9Z0Z',
    'Test Landlord',
    '123 Test Street, Paris 75001',
    '+33 1 23 45 67 89'
)
ON CONFLICT (id) DO NOTHING;
