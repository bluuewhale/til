import { Router } from 'express';

const router = Router();

const VERSION = '0.0.1';

router.get('/', (_req, res) => res.json({ version: VERSION }));

export default router;
