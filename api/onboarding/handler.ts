import type { VercelRequest, VercelResponse } from '@vercel/node';

export default async (req: VercelRequest, res: VercelResponse) => {

    // do something with req and res
    
    return res.status(200).send('ok');
}