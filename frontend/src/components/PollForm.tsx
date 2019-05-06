import React from 'react';
import TextField from '@material-ui/core/TextField';
import InputAdornment from '@material-ui/core/InputAdornment';
import FormControlLabel from '@material-ui/core/FormControlLabel';
import Checkbox from '@material-ui/core/Checkbox';
import IconButton from '@material-ui/core/IconButton';
import DeleteIcon from '@material-ui/icons/Delete';
import Button from '@material-ui/core/Button';
import FormGroup from '@material-ui/core/FormGroup';
import Paper from '@material-ui/core/Paper';
import { Machine } from 'xstate';

interface PollFormSchema {
    states: {
        dataEntry: {};
        submitting: {};
    };
}

interface PollFormContext {
    name: string;
}

type PollFormEvent = { type: 'ENTER_NAME' } | { type: 'SUBMIT' };

const pollFormMachine = Machine<PollFormContext, PollFormSchema, PollFormEvent>(
    {
        key: 'pollForm',
        initial: 'dataEntry',
        context: { name: '' },
        states: {
            dataEntry: {
                on: {
                    ENTER_NAME: 'asdf',
                },
            },
            submitting: {},
        },
    },
);

export default function PollForm() {
    return (
        <form>
            <Paper>
                <TextField
                    placeholder='What would you like to ask?'
                    margin='normal'
                    variant='outlined'
                    autoFocus
                    fullWidth
                />
                <TextField
                    variant='outlined'
                    fullWidth
                    margin='dense'
                    placeholder='Option 1'
                    InputProps={{
                        endAdornment: (
                            <InputAdornment position='end'>
                                <IconButton aria-label='Delete' edge='end'>
                                    <DeleteIcon fontSize='small' />
                                </IconButton>
                            </InputAdornment>
                        ),
                    }}
                />
                <TextField
                    variant='outlined'
                    fullWidth
                    margin='dense'
                    placeholder='Option 2'
                    InputProps={{
                        endAdornment: (
                            <InputAdornment position='end'>
                                <IconButton aria-label='Delete' edge='end'>
                                    <DeleteIcon fontSize='small' />
                                </IconButton>
                            </InputAdornment>
                        ),
                    }}
                />
                <FormGroup row>
                    <FormControlLabel
                        control={<Checkbox value='checkedB' color='primary' />}
                        label='Allow write-in answers'
                    />
                    <FormControlLabel
                        control={<Checkbox value='checkedB' color='primary' />}
                        label='Allow multiple answers'
                    />
                </FormGroup>
            </Paper>

            <Button
                variant='contained'
                color='primary'
                size='large'
                type='submit'
            >
                Create Poll
            </Button>
        </form>
    );
}
