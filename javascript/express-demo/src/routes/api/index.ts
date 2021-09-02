import { Router } from 'express';

import versionRouter from './versions';
import authRouter from './auth';
import movieRouter from './movies';

const router = Router();

router.use('/version', versionRouter);
router.use('/auth', authRouter);
router.use('/movies', movieRouter);

export default router;
