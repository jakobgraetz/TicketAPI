-- Enable Row Level Security
ALTER TABLE public.keys ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.tickets ENABLE ROW LEVEL SECURITY;

-- Create keys table
CREATE TABLE public.keys (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    api_key TEXT NOT NULL,
    name TEXT NOT NULL,
    preview TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used TIMESTAMPTZ,
    total_uses INTEGER NOT NULL DEFAULT 0
);

-- Create tickets table
CREATE TABLE public.tickets (
    id BIGSERIAL PRIMARY KEY,
    key_id BIGINT NOT NULL REFERENCES public.keys(id),
    event_name TEXT,
    event_date TIMESTAMPTZ,
    event_location TEXT,
    holder_name TEXT,
    holder_email TEXT,
    status TEXT,
    notes TEXT,
    terms_and_conditions TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);

-- Create index on foreign key
CREATE INDEX idx_tickets_key_id ON public.tickets(key_id);

-- Row Level Security policies for keys table
CREATE POLICY "Users can view their own keys"
    ON public.keys FOR SELECT
    USING (auth.uid() = user_id);

CREATE POLICY "Users can insert their own keys"
    ON public.keys FOR INSERT
    WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can update their own keys"
    ON public.keys FOR UPDATE
    USING (auth.uid() = user_id);

CREATE POLICY "Users can delete their own keys"
    ON public.keys FOR DELETE
    USING (auth.uid() = user_id);

-- Row Level Security policies for tickets table
CREATE POLICY "Users can view tickets associated with their keys"
    ON public.tickets FOR SELECT
    USING (EXISTS (
        SELECT 1 FROM public.keys
        WHERE keys.id = tickets.key_id AND keys.user_id = auth.uid()
    ));

CREATE POLICY "Users can insert tickets associated with their keys"
    ON public.tickets FOR INSERT
    WITH CHECK (EXISTS (
        SELECT 1 FROM public.keys
        WHERE keys.id = tickets.key_id AND keys.user_id = auth.uid()
    ));

CREATE POLICY "Users can update tickets associated with their keys"
    ON public.tickets FOR UPDATE
    USING (EXISTS (
        SELECT 1 FROM public.keys
        WHERE keys.id = tickets.key_id AND keys.user_id = auth.uid()
    ));

CREATE POLICY "Users can delete tickets associated with their keys"
    ON public.tickets FOR DELETE
    USING (EXISTS (
        SELECT 1 FROM public.keys
        WHERE keys.id = tickets.key_id AND keys.user_id = auth.uid()
    ));

-- Create a function to update the updated_at column
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
}
$$ LANGUAGE plpgsql;

-- Create a trigger to automatically update the updated_at column
CREATE TRIGGER update_tickets_modtime
    BEFORE UPDATE ON public.tickets
    FOR EACH ROW
    EXECUTE FUNCTION update_modified_column();

