import Typography from '@material-ui/core/Typography';
import classNames from 'classnames';
import React from 'react';
import { makeStyles } from '../styles';

export type Props<E> = React.DetailedHTMLProps<React.HTMLAttributes<E>, E>;

const useHeaderStyles = makeStyles((theme) => ({
    header: {
        minHeight: '150px',
        display: 'flex',
        flexDirection: 'column',
        justifyContent: 'center',
        alignItems: 'center',
    },
}));

export function Header(props: Props<HTMLElement>) {
    const classes = useHeaderStyles();
    return (
        <header
            {...props}
            className={classNames(classes.header, props.className)}
        >
            {props.children}
        </header>
    );
}

export interface TitleProps extends Props<HTMLElement> {
    readonly gutterBottom?: boolean;
}

export function Title(props: TitleProps) {
    return (
        <Typography
            variant='h4'
            align='center'
            gutterBottom={props.gutterBottom}
        >
            {props.children}
        </Typography>
    );
}

export function Subtitle(props: TitleProps) {
    return (
        <Typography
            variant='body1'
            align='center'
            gutterBottom={props.gutterBottom}
        >
            {props.children}
        </Typography>
    );
}
