import React from 'react';
import * as Site from './Site';
import Typography from '@material-ui/core/Typography';
import Container from '@material-ui/core/Container';
import Grid from '@material-ui/core/Grid';
import Fade from '@material-ui/core/Fade';
import Box from '@material-ui/core/Box';
import PollForm from './PollForm';

export default function HomePage() {
    return (
        <Site.Skeleton>
            <Container maxWidth='lg'>
                <Box m={4}>
                    <Typography variant='h4' gutterBottom align='center'>
                        Real-time polls on EOSIO blockchains
                    </Typography>
                </Box>
                <Grid container spacing={5}>
                    <Grid item xs>
                        <div>Popular polls</div>
                    </Grid>
                    <Grid item xs={6}>
                        <PollForm />
                    </Grid>
                    <Grid item xs>
                        <div>Recent polls</div>
                    </Grid>
                </Grid>
            </Container>
        </Site.Skeleton>
    );
}
