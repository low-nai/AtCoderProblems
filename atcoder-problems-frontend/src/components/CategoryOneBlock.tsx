import * as React from "react";
import { Row, PageHeader } from "react-bootstrap";
import { BootstrapTable, TableHeaderColumn } from "react-bootstrap-table";
import { Problem } from "../model/Problem";
import { Contest } from "../model/Contest";
import { some, none, Option } from "ts-option";
import { UrlFormatter } from "../utils/UrlFormatter";
import { HtmlFormatter } from "../utils/HtmlFormatter";

export interface CategoryOneBlockProps {
    categoryTitle: string;
    data: Array<[Contest, Array<Problem>]>;
    header: Array<string>;
}

export class CategoryOneBlock extends React.Component<CategoryOneBlockProps, {}>{

    private contestLinkFormatter(contest: Contest) {
        return HtmlFormatter.createLink(UrlFormatter.contestUrl(contest), contest.id.toUpperCase());
    }

    private problemLinkFormatter(problem: Option<Problem>, row: { [key: string]: any }) {
        let contest = row["contest"];

        return problem.match({
            some: p => HtmlFormatter.createLink(UrlFormatter.problemUrl(contest, p), p.title),
            none: () => (<span>-</span>)
        });
    }

    private columnColorFormatter(problem: Option<Problem>): string {
        return problem.match({
            some: p => {
                return "danger";
            },
            none: () => ""
        });
    }

    render() {
        let data = this.props.data.map(d => {
            let contest = d[0];
            let problems = d[1];
            let o: { [key: string]: any } = { contest: d[0] };
            this.props.header.forEach((head, i) => {
                if (problems.length > i) {
                    o[head] = some(problems[i]);
                } else {
                    o[head] = none;
                }
            });
            return o;
        });

        return (
            <Row>
                <PageHeader>{this.props.categoryTitle}</PageHeader>
                <BootstrapTable data={data}>
                    <TableHeaderColumn dataField="contest" isKey dataFormat={this.contestLinkFormatter}>
                        Contest
                    </TableHeaderColumn>
                    {
                        this.props.header.map((head, i) =>
                            (
                                <TableHeaderColumn key={i} dataField={head} dataFormat={this.problemLinkFormatter} columnClassName={this.columnColorFormatter}>
                                    {head}
                                </TableHeaderColumn>
                            )
                        )
                    }
                </BootstrapTable>
            </Row>);
    }
}