import React from 'react';
import * as Site from './Site';
import Typography from '@material-ui/core/Typography';
import Container from '@material-ui/core/Container';
import Grid from '@material-ui/core/Grid';
import Fade from '@material-ui/core/Fade';
import PollForm from './PollForm';

export default function HomePage() {
    return (
        <Site.Skeleton>
            <Container maxWidth='lg'>
                <Typography variant='h3' gutterBottom align='center'>
                    Real-time polls on EOS blockchains
                </Typography>
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
